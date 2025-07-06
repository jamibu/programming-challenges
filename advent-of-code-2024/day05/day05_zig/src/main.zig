const std = @import("std");
const print = std.debug.print;

const Rule = struct {
    before: usize,
    after: usize,
};

const PageOrder = struct {
    index: std.AutoHashMap(usize, usize),
    rules: std.ArrayList(Rule),
    lastIdx: usize,

    pub fn init() PageOrder {
        var index = std.AutoHashMap(usize, usize).init(std.heap.page_allocator);
        var rules = std.ArrayList(Rule).init(std.heap.page_allocator);
        defer index.deinit();
        defer rules.deinit();
        return PageOrder{ .index = index, .rules = rules, .lastIdx = 0 };
    }

    pub fn addRule(self: *PageOrder, rule: Rule) !void {
        try self.rules.append(rule);

        const beforeIdx = try self.getOrPut(rule.before);
        const afterIdx = try self.getOrPut(rule.after);

        if (beforeIdx > afterIdx) {
            try self.index.put(rule.before, afterIdx);
            try self.index.put(rule.after, beforeIdx);
        }
    }

    pub fn revaluateRules(self: *PageOrder) !void {
        for (self.rules.items) |rule| {
            try self.evaluateRule(rule);
        }
    }

    pub fn evaluateRule(self: *PageOrder, rule: Rule) !void {
        const beforeIdx = self.index.get(rule.before).?;
        const afterIdx = self.index.get(rule.after).?;

        if (beforeIdx > afterIdx) {
            try self.index.put(rule.before, afterIdx);
            try self.index.put(rule.after, beforeIdx);
        }
    }

    fn getOrPut(self: *PageOrder, key: usize) !usize {
        const v = try self.index.getOrPut(key);
        if (!v.found_existing) {
            // We inserted an entry, specify the new value
            // This is a conditional in case creating the new value is expensive
            self.lastIdx += 1;
            v.value_ptr.* = self.lastIdx;
        }

        return v.value_ptr.*;
    }

    fn printOrder(self: *const PageOrder) void {
        var pageIter = self.index.iterator();
        while (pageIter.next()) |page| {
            print("page: {d}, idx: {d}\n", .{ page.key_ptr.*, page.value_ptr.* });
        }
    }
};

pub fn main() !void {
    // Build dict of index based on rules
    // - Is first in index?
    //      - n add at end
    //      - compare index, swap if needed
    // - Is first in index?
    //      - n add at end
    //
    // 47 | 53 = {47: 0, 53: 1}
    // 97 | 13 = {47: 0, 53: 1, 97: 2, 13: 3}
    // 97 | 61 = {47: 0, 53: 1, 97: 2, 13: 3, 61: 4}
    // 97 | 47 = {97: 0, 53: 1, 47: 2, 13: 3, 61: 4}
    // 75 | 29 = {97: 0, 53: 1, 47: 2, 13: 3, 61: 4, 75: 5, 29: 6}
    // 61 | 13 = {97: 0, 53: 1, 47: 2, 61: 3, 13: 4, 75: 5, 29: 6}
    // 75 | 53 = {97: 0, 75: 1, 47: 2, 61: 3, 13: 4, 53: 5, 29: 6}
    // 29 | 13 = {97: 0, 75: 1, 47: 2, 61: 3, 29: 4, 53: 5, 13: 6}
    //
}

fn readFile(fname: []const u8) !struct { pageOrder: PageOrder, updates: std.ArrayList([]const u8) } {
    var file = try std.fs.cwd().openFile(fname, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;

    var pageOrder = PageOrder.init();
    var updates = std.ArrayList([]const u8).init(std.heap.page_allocator);
    var readingRules: bool = true;

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (std.mem.eql(u8, line, "")) {
            readingRules = false;
            continue;
        }

        if (readingRules) {
            var splitRule = std.mem.tokenizeScalar(u8, line, '|');
            const before = try std.fmt.parseInt(usize, splitRule.next().?, 10);
            const after = try std.fmt.parseInt(usize, splitRule.next().?, 10);

            try pageOrder.addRule(Rule{ .before = before, .after = after });
        } else {
            try updates.append(line);
        }
    }

    return .{ .pageOrder = pageOrder, .updates = updates };
}

test "Test Read" {
    const pageOrder = try readFile("../input.txt");
    var order = pageOrder.pageOrder;
    // const updates = pageOrder.updates;
    // defer order.deinit();
    // defer updates.deinit();
    order.printOrder();

    for (order.rules.items) |rule| {
        try order.evaluateRule(rule);

        const beforeIdx = order.index.get(rule.before).?;
        const afterIdx = order.index.get(rule.after).?;
        print("{any}", .{rule});
        print("{any}", .{rule});

        try std.testing.expect(beforeIdx < afterIdx);
    }

    for (order.rules.items) |rule| {
        const beforeIdx = order.index.get(rule.before).?;
        const afterIdx = order.index.get(rule.after).?;
        print("{any}", .{rule});
        print("{any}", .{rule});

        try std.testing.expect(beforeIdx < afterIdx);
    }
}

// test "Test example" {
//     const pageOrder = try readFile("../example.txt");
//     const order = pageOrder.pageOrder;
//     // const updates = pageOrder.updates;
//     // defer order.index.deinit();
//     // defer order.rules.deinit();
//     // defer updates.deinit();
//
//     for (order.rules.items) |rule| {
//         const beforeIdx = order.index.get(rule.before).?;
//         const afterIdx = order.index.get(rule.after).?;
//
//         try std.testing.expect(beforeIdx < afterIdx);
//     }
// }

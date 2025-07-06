const std = @import("std");
const root = @import("root.zig");
const mul = "mul";

const Mul = struct {
    num1: i32,
    num2: i32,
};

pub fn main() !void {
    const ans = try solve("../input.txt");
    std.debug.print("Part 1: {d}", .{ans});
}

fn solve(fname: []const u8) !i32 {
    var file = try std.fs.cwd().openFile(fname, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;
    var mulList = std.ArrayList(Mul).init(std.heap.page_allocator);
    while (try in_stream.readUntilDelimiterOrEof(&buf, ')')) |line| {
        if (line.len < 7) {
            continue;
        }

        try parseTokens(line, &mulList);
    }

    std.debug.print("{any}\n", .{mulList.items});

    var ans: i32 = 0;
    for (mulList.items) |m| {
        ans += m.num1 * m.num2;
    }

    return ans;
}

fn parseTokens(line: []u8, mulList: *std.ArrayList(Mul)) !void {
    var list = std.ArrayList(root.Token).init(std.heap.page_allocator);
    defer list.deinit();

    var lexer = root.Lexer.new(line);

    while (lexer.nextToken()) |tok| {
        try list.append(tok);
    }

    if (list.items.len < 5) {
        return;
    }

    const last5 = list.items[list.items.len - 5 .. list.items.len];
    std.debug.print("Line - {s}\n", .{line});
    std.debug.print("Last 5 tokens - {any}\n\n", .{last5});

    const isMul = last5[0].type == root.TokenType.identifier and std.mem.eql(u8, last5[0].literal, "mul");
    const isLparen = last5[1].type == root.TokenType.lparen;
    const isNum1 = last5[2].type == root.TokenType.number;
    const isComma = last5[3].type == root.TokenType.comma;
    const isNum2 = last5[4].type == root.TokenType.number;

    if (isMul and isLparen and isNum1 and isComma and isNum2) {
        const num1 = try std.fmt.parseInt(i32, last5[2].literal, 10);
        const num2 = try std.fmt.parseInt(i32, last5[4].literal, 10);
        try mulList.append(Mul{ .num1 = num1, .num2 = num2 });
    }
}

test "Example 1" {
    const ans = try solve("../example.txt");
    std.debug.print("Got: {d}\n", .{ans});
    try std.testing.expect(ans == 161);
}

const std = @import("std");
const print = std.debug.print;
const parseInt = std.fmt.parseInt;

const Locations = struct {
    left: []i32,
    right: []i32,
};

pub fn main() !void {
    const vals = try readFileLines("../input.txt");

    const part1Ans = part1(vals);
    const part2Ans = part2(vals);

    print("Part 1: {d}\n", .{part1Ans});
    print("Part 2: {d}\n", .{part2Ans});
}

pub fn part1(vals: Locations) u32 {
    var diffSum: u32 = 0;

    std.mem.sort(i32, vals.left, {}, std.sort.asc(i32));
    std.mem.sort(i32, vals.right, {}, std.sort.asc(i32));

    for (vals.right, 0..) |val, index| {
        diffSum += @abs(val - vals.left[index]);
    }

    return diffSum;
}

pub fn part2(vals: Locations) i32 {
    var similarityScore: i32 = 0;

    var idCountMap = std.AutoHashMap(i32, i32).init(std.heap.page_allocator);
    defer idCountMap.deinit();

    for (vals.right) |id| {
        const value = idCountMap.get(id) orelse 0;
        idCountMap.put(id, value + 1) catch unreachable;
    }

    for (vals.left) |id| {
        if (idCountMap.get(id)) |v| {
            similarityScore += id * v;
        }
    }

    return similarityScore;
}

fn readFileLines(filePath: []const u8) !Locations {
    var leftVals = std.ArrayList(i32).init(std.heap.page_allocator);
    var rightVals = std.ArrayList(i32).init(std.heap.page_allocator);

    var file = try std.fs.cwd().openFile(filePath, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var row = std.mem.tokenizeScalar(u8, line, ' ');

        const leftStr = row.next().?;
        const rightStr = row.next().?;

        try leftVals.append(try std.fmt.parseInt(i32, leftStr, 10));
        try rightVals.append(try std.fmt.parseInt(i32, rightStr, 10));
    }

    return .{ .left = leftVals.items, .right = rightVals.items };
}

test "Part1 Example" {
    const vals = try readFileLines("../example.txt");
    const part1Ans = part1(vals);

    try std.testing.expect(part1Ans == 11);
}

test "Part2 Example" {
    const vals = try readFileLines("../example.txt");
    const part2Ans = part2(vals);

    try std.testing.expect(part2Ans == 31);
}

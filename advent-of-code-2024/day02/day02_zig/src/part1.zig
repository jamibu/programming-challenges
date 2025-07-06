const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    var reportIterator = try root.ReportIterator.init("../input.txt");
    const ans = try part1(&reportIterator);

    std.debug.print("Part1: {d}\n", .{ans});
}

pub fn part1(reportIterator: *root.ReportIterator) !usize {
    var ans: usize = 0;
    while (try reportIterator.next()) |report| {
        ans += if (checkReport(report)) 1 else 0;
    }

    return ans;
}

fn checkReport(report: []i32) bool {
    // Check if all levels are save
    const isIncreasing = report[0] < report[report.len - 1];
    for (0..report.len - 1) |i| {
        if (isIncreasing != (report[i] < report[i + 1])) {
            return false;
        }
        const diff = @abs(report[i + 1] - report[i]);
        if (diff < 1 or diff > 3) {
            return false;
        }
    }

    return true;
}

test "Example" {
    var reportIterator = try root.ReportIterator.init("../example.txt");
    const ans = try part1(&reportIterator);

    std.debug.print("Part1: {d}\n", .{ans});
    try std.testing.expect(ans == 2);
}

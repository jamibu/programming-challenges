const std = @import("std");
const print = std.debug.print;
const nrows = 140;
const ncols = 140;
const filepath = "../input.txt";

fn Matrix(
    comptime T: type,
    comptime width: comptime_int,
    comptime height: comptime_int,
) type {
    return [height][width]T;
}

const horizontal: Matrix(usize, 4, 2) = .{
    .{ 0, 1, 2, 3 },
    .{ 0, 0, 0, 0 },
};

const vertical: Matrix(usize, 4, 2) = .{
    .{ 0, 0, 0, 0 },
    .{ 0, 1, 2, 3 },
};

const diagonal1: Matrix(usize, 4, 2) = .{
    .{ 0, 1, 2, 3 },
    .{ 0, 1, 2, 3 },
};

const diagonal2: Matrix(usize, 4, 2) = .{
    .{ 0, 1, 2, 3 },
    .{ 3, 2, 1, 0 },
};

pub fn main() !void {}

fn searchGrid(grid: Matrix(u8, nrows, ncols)) usize {
    var count: usize = 0;
    for (0..nrows) |i| {
        for (0..ncols) |j| {
            if (j < ncols - 3 and i < nrows - 3) {
                // print("{s}\n", .{"All"});
                count += if (checkXmas(grid, diagonal1, i, j)) 1 else 0;
                count += if (checkXmas(grid, diagonal2, i, j)) 1 else 0;
                count += if (checkXmas(grid, horizontal, i, j)) 1 else 0;
                count += if (checkXmas(grid, vertical, i, j)) 1 else 0;
            } else if (j < ncols - 3) {
                // print("{s}\n", .{"horizontal"});
                count += if (checkXmas(grid, horizontal, i, j)) 1 else 0;
            } else if (i < nrows - 3) {
                // print("{s}\n", .{"vertical"});
                count += if (checkXmas(grid, vertical, i, j)) 1 else 0;
            }
        }
    }

    return count;
}

fn checkXmas(
    grid: Matrix(u8, nrows, ncols),
    direction: Matrix(usize, 4, 2),
    row: usize,
    col: usize,
) bool {
    var string: [4]u8 = undefined;
    for (0..4) |i| {
        string[i] = grid[row + direction[1][i]][col + direction[0][i]];
    }
    print("{s}\n", .{string});

    return std.mem.eql(u8, &string, "XMAS") or std.mem.eql(u8, &string, "SAMX");
}

fn readFile(fname: []const u8) !Matrix(u8, nrows, ncols) {
    var file = try std.fs.cwd().openFile(fname, .{});
    defer file.close();

    var grid: Matrix(u8, nrows, ncols) = undefined;
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;
    var row: usize = 0;

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        for (line, 0..) |l, i| {
            grid[row][i] = l;
        }
        row += 1;
    }

    return grid;
}

test "Part1" {
    const grid = try readFile(filepath);
    const ans = searchGrid(grid);
    std.debug.print("Got: {d}\n", .{ans});
    try std.testing.expect(ans == 18);
}

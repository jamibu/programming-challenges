const std = @import("std");
const print = std.debug.print;

// const nrows = 10;
// const ncols = 10;
// const filepath = "../example.txt";

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

const diagonal1: Matrix(usize, 3, 2) = .{
    .{ 0, 1, 2 },
    .{ 0, 1, 2 },
};

const diagonal2: Matrix(usize, 3, 2) = .{
    .{ 0, 1, 2 },
    .{ 2, 1, 0 },
};

pub fn main() !void {}

fn searchGrid(grid: Matrix(u8, nrows, ncols)) usize {
    var count: usize = 0;
    for (0..nrows - 2) |i| {
        for (0..ncols - 2) |j| {
            const isXmas: bool = checkXmas(grid, i, j);
            count += if (isXmas) 1 else 0;
        }
    }

    return count;
}

fn checkXmas(
    grid: Matrix(u8, nrows, ncols),
    row: usize,
    col: usize,
) bool {
    var string1: [3]u8 = undefined;
    var string2: [3]u8 = undefined;
    for (0..3) |i| {
        string1[i] = grid[row + diagonal1[1][i]][col + diagonal1[0][i]];
        string2[i] = grid[row + diagonal2[1][i]][col + diagonal2[0][i]];
    }

    const isMas1 = std.mem.eql(u8, &string1, "MAS") or std.mem.eql(u8, &string1, "SAM");
    const isMas2 = std.mem.eql(u8, &string2, "MAS") or std.mem.eql(u8, &string2, "SAM");

    return isMas1 and isMas2;
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
    try std.testing.expect(ans == 9);
}

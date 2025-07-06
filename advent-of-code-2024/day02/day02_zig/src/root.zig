const std = @import("std");

const InStreamType = std.io.BufferedReader(4096, std.fs.File.Reader).Reader;

pub const ReportIterator = struct {
    file: std.fs.File,
    inStream: InStreamType,
    buf: [1024]u8,

    pub fn init(fname: []const u8) !ReportIterator {
        var file = try std.fs.cwd().openFile(fname, .{});
        var bufReader = std.io.bufferedReader(file.reader());

        return ReportIterator{
            .file = file,
            .inStream = bufReader.reader(),
            .buf = undefined,
        };
    }

    pub fn deinit(self: *ReportIterator) void {
        self.file.close();
    }

    pub fn next(self: *ReportIterator) !?[]i32 {
        if (try self.inStream.readUntilDelimiterOrEof(&self.buf, '\n')) |line| {
            var list = std.ArrayList(i32).init(std.heap.page_allocator);
            var vals = std.mem.tokenizeScalar(u8, line, ' ');
            while (vals.next()) |val| {
                try list.append(try std.fmt.parseInt(i32, val, 10));
            }
            return list.items;
        }

        return null;
    }
};

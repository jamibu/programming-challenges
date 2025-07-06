const std = @import("std");
const testing = std.testing;

pub const TokenType = enum { illegal, rparen, lparen, comma, identifier, number, eof };

pub const Token = struct { type: TokenType, literal: []u8 };

pub const Lexer = struct {
    input: []u8,
    position: usize,
    readPosition: usize,
    ch: ?u8,

    pub fn new(input: []u8) Lexer {
        return Lexer{
            .input = input,
            .position = 0,
            .readPosition = 1,
            .ch = input[0],
        };
    }

    pub fn nextToken(self: *Lexer) ?Token {
        const ch = self.ch orelse return null;
        const chLiteral = self.input[self.position .. self.position + 1];

        const tok = switch (ch) {
            '(' => Token{ .type = TokenType.lparen, .literal = chLiteral },
            ')' => Token{ .type = TokenType.rparen, .literal = chLiteral },
            ',' => Token{ .type = TokenType.comma, .literal = chLiteral },
            'm' => Token{ .type = TokenType.identifier, .literal = self.readIdentifier() },
            'd' => Token{ .type = TokenType.identifier, .literal = self.readIdentifier() },
            '0'...'9' => Token{ .type = TokenType.number, .literal = self.readNumber() },
            else => Token{ .type = TokenType.illegal, .literal = chLiteral },
        };

        if (tok.type != TokenType.number and tok.type != TokenType.identifier) {
            self.readChar();
        }

        return tok;
    }

    fn readChar(self: *Lexer) void {
        if (self.position >= self.input.len - 1) {
            self.ch = null;
        } else {
            self.ch = self.input[self.readPosition];
        }
        self.position = self.readPosition;
        self.readPosition += 1;
    }

    fn readNumber(self: *Lexer) []u8 {
        const start = self.position;
        while (isDigit(self.ch)) {
            self.readChar();
        }

        return self.input[start..self.position];
    }

    fn readIdentifier(self: *Lexer) []u8 {
        const start = self.position;
        while (isLetter(self.ch)) {
            self.readChar();
        }
        return self.input[start..self.position];
    }
};

pub fn isLetter(ch: ?u8) bool {
    const char = ch orelse return false;
    return ('a' <= char and char <= 'z') or char == '\'';
}

pub fn isDigit(ch: ?u8) bool {
    const char = ch orelse return false;
    return '0' <= char and char <= '9';
}

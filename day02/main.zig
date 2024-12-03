const std = @import("std");

const ZIG_MAGIC_NUMBER = -1431655766;

fn set_zero(input: []u8) void {
    for (0..input.len) |i| {
        input[i] = 0;
    }
}

fn remove_index(arr: []const i32, index: usize) [10]i32 {
    var buf: [10]i32 = undefined;
    var buf_tail: usize = 0;

    for (arr, 0..) |value, i| {
        if (i == index) {
            continue;
        }
        buf[buf_tail] = value;
        buf_tail += 1;
    }

    return buf;
}

fn parse(input: []const u8) ![10]i32 {
    var buf: [3]u8 = undefined;
    var buf_tail: usize = 0;

    var nums: [10]i32 = undefined;
    var nums_tail: usize = 0;
    for (input, 0..) |ch, i| {
        if (ch == ' ' or i + 1 == input.len) {
            const num = try std.fmt.parseInt(i32, buf[0..buf_tail], 10);
            buf_tail = 0;
            set_zero(buf[0..]);

            nums[nums_tail] = num;
            nums_tail += 1;

            if (ch == ' ') {
                continue;
            }
            break;
        }

        buf[buf_tail] = ch;
        buf_tail += 1;
    }

    return nums;
}

fn is_safe(numbers: []const i32) bool {
    var is_inc = false;
    var is_dec = false;
    var prev_num: i32 = 0;
    for (numbers) |num| {
        if (num == ZIG_MAGIC_NUMBER) {
            break;
        }

        if (num > prev_num) {
            if (is_dec and prev_num != 0) {
                return false;
            }

            if (prev_num != 0) {
                is_inc = true;
            }
        } else if (num < prev_num) {
            if (is_inc and prev_num != 0) {
                return false;
            }

            if (prev_num != 0) {
                is_dec = true;
            }
        } else {
            return false;
        }

        const diff = @abs(prev_num - num);
        if (diff > 3 and prev_num != 0) {
            return false;
        }

        prev_num = num;
    }

    return true;
}

pub fn main() !void {
    // part 1
    const file = try std.fs.cwd().openFile("./input.txt", .{});
    defer file.close();

    var reader = std.io.bufferedReader(file.reader());
    var in_stream = reader.reader();

    var buf: [100]u8 = undefined;
    var total_safe: u32 = 0;
    var total_safe_damp: u32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const nums = try parse(line);
        const safe = is_safe(nums[0..]);

        // part 1
        if (safe) {
            total_safe += 1;
        }
        // part 2
        if (safe) {
            total_safe_damp += 1;
        } else {
            for (0..nums.len) |i| {
                const new_nums = remove_index(nums[0..], i);
                if (is_safe(new_nums[0..])) {
                    total_safe_damp += 1;
                    break;
                }
            }
        }
    }

    std.debug.print("safe: {}\n", .{total_safe});
    std.debug.print("safe with damp: {}\n", .{total_safe_damp});
}

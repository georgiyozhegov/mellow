#include "debug.h"

extern void write_c(char value) {
      __asm__ (
            "mov $1, %%rax\n"
            "mov $1, %%rdi\n"
            "lea %0, %%rsi\n"
            "mov $1, %%rdx\n"
            "syscall"
            : : "m"(value)
            : "rax", "rdi", "rsi", "rdx"
      );
}

extern void debug_c(char value) {
      write_c(value);
      write_c('\n');
}

extern void write_i64(long long value) {
      if (value == 0) {
            write_c('0');
            return;
      }

      char buffer[20];
      int size = 0;
      int negative = value < 0;

      if (negative)
            value = -value;

      while (value > 0) {
            int digit = value % 10;
            buffer[size++] = digit + '0';
            value = (value - digit) / 10;
      }

      if (negative)
            buffer[size++] = '-';

      for (int index = size - 1; index >= 0; index--)
            write_c(buffer[index]);
}

extern void debug_i64(long long value) {
      write_i64(value);
      write_c('\n');
}

extern void write_s(char *value) {
      for (char *c = value; *c != 0; c++)
            write_c(*c);
}

extern void debug_s(char *value) {
      write_s(value);
      write_c('\n');
}

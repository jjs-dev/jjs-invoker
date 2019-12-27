#include "tests.h"
#include <stdio.h>
#include <stdlib.h>
static void exceed_time_limit() {
    while (1) {
        // just some staff with side effects
        write(-1 /*invalid FD*/, "", 0);
    }
}
void test_tl() { exceed_time_limit(); }

void test_tl_fork() {
    fork();
    exceed_time_limit();
}

void test_il() { usleep(10000000); }

void test_abort() { abort(); }

void test_return_1() { exit(1); }

void test_ok() { exit(0); }

void test_consume_memory() {
    while (1) {
        // alloc one MiB
        size_t const allocation_size = ((size_t) 1) << 20;
        char* ptr = (char*) malloc(allocation_size);
        if (ptr == NULL) {
            break;
        }
        size_t const page_size = 4096;
        for (volatile char* p = ptr; p < ptr + allocation_size;
             p += page_size) {
            *p = 228;
        }
    }
    exit(0);
}

const struct test tests[] = {
    {"tl", test_tl, "TL\n", 1, 2},
    {"tl_fork", test_tl_fork, "TL\n", 1, 2},
    {"il", test_il, "ILE\n", 1, 2},
    {"abort", test_abort, "exit code -6\n", 1, 2},
    {"return1", test_return_1, "exit code 1\n", 1, 2},
    {"ok", test_ok, "exit code 0\n", 1, 2},
    {"consume_memory", test_consume_memory, "exit code -9\n", 1, 2},
    {"wait_timeout", test_il, "Wait timed out\n", 1, 10},
    {NULL, NULL, NULL}};

#include "esp_log.h"

struct Pair {
    int a, b;
};

void abi_test_5(int p1, int p2, int p3, int p4, struct Pair p5) {
    ESP_LOGI("abi_test_5", "p5.a=%d, p5.b=%d", p5.a, p5.b);
}

void abi_test_6(int p1, int p2, int p3, int p4, int p5, struct Pair p6) {
    ESP_LOGI("abi_test_6", "p6.a=%d, p6.b=%d", p6.a, p6.b);
}

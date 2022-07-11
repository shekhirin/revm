#include "intx.h"

struct RetUint 
{
    uint64_t n1;
    uint64_t n2;
    uint64_t n3;
    uint64_t n4;
};

extern "C" RetUint fast_div_rem(uint64_t* first, uint64_t* second) {
    intx::uint256 f = intx::uint256(first[0],first[1],first[2],first[3]);
    intx::uint256 s = intx::uint256(second[0], second[1], second[2], second[3]);
    auto t = f / s;
    struct RetUint ret{t[0], t[1], t[2], t[3]};
    return ret;
} 
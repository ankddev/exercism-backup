#include "difference_of_squares.h"

// The function sum_of_squares() calculates the sum of the squares of the first number natural numbers.
unsigned int sum_of_squares(unsigned int number) {
    unsigned int sum = 0;
    for (unsigned int i = 1; i <= number; i++) {
        sum += i * i;
    }
    return sum;
}

// The function square_of_sum() calculates the square of the sum of the first number natural numbers.
unsigned int square_of_sum(unsigned int number) {
    unsigned int sum = 0;
    for (unsigned int i = 1; i <= number; i++) {
        sum += i;
    }
    return sum * sum;
}

// The function difference_of_squares() calculates the difference between the square of the sum of the first number natural numbers and the sum of the squares of the first number natural numbers.
unsigned int difference_of_squares(unsigned int number) {
    return square_of_sum(number) - sum_of_squares(number);
}

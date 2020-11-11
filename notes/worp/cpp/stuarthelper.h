

#include <iostream>
//#ifndef CROW_ALL_STUARTHELPER_H
//#define CROW_ALL_STUARTHELPER_H
//#endif //CROW_ALL_STUARTHELPER_H

using namespace std;

#include <chrono>

void printArray(int array[], int len) {
    for (int i = 0; i < len; i++) {
        cout << array[i] << " ";
    }
    cout << endl;
}


void printIntersection(int x[], int y[], int lx, int ly) {
    //https://www.geeksforgeeks.org/union-and-intersection-of-two-sorted-arrays-2/

    int maxToPrint = 100;
    printArray(x, maxToPrint);
    printArray(y, maxToPrint);
    int* intersection = new int[lx];
    int intersection_i = 0;
    chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
    int i = 0, j = 0;
    while (i < lx && j < ly) {
        if (x[i] < y[j])
            i++;
        else if (y[j] < x[i])
            j++;
        else                                                // x[i] == y[j]
        {
//            cout << y[j] << " ";
            intersection[intersection_i++] = y[j];
            i++;
            j++;
        }
    }
    cout << endl;

    chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    printArray(intersection, maxToPrint);
    cout << endl;


    std::cout << chrono::duration_cast<std::chrono::milliseconds>(end - begin).count() << "[ms]" << std::endl;
    std::cout << chrono::duration_cast<std::chrono::microseconds>(end - begin).count() << "[µs]" << std::endl;
    std::cout << chrono::duration_cast<std::chrono::nanoseconds>(end - begin).count() << "[ns]" << std::endl;
}

void runPrintIntersection() {


    int arr1[] = {1, 2, 4, 5, 6};
    int arr2[] = {2, 3, 5, 7};

    int lx = sizeof(arr1) / sizeof(arr1[0]);
    int ly = sizeof(arr2) / sizeof(arr2[0]);

    printIntersection(arr1, arr2, lx, ly);

}

int *getRandomSortedArrayOfLength(int length) {
    //http://www.cplusplus.com/forum/beginner/5527/
    //https://stackoverflow.com/questions/3473438/return-array-in-a-function
    //https://www.geeksforgeeks.org/return-local-array-c-function/
    //https://stackoverflow.com/questions/19894686/getting-size-of-array-from-pointer-c
    int *array = new int[length];
    int diff;

//    srand((unsigned) time(0));

    array[0] = 0;

    for (int i = 1; i < length; i++) {
        diff = rand() % 4 + 1;
        array[i] = array[i - 1] + diff;
    }
    return array;
}

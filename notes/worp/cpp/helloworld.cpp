using namespace std;

#include "crow.h"
#include "stuart/stuarthelper.h"


int main() {

    int length = 1000111;

    int lx = length/4;
    int ly = length;

//    printArray(getRandomSortedArrayOfLength(length), length);

    srand((unsigned) time(nullptr));

    int* x = getRandomSortedArrayOfLength(lx);
    int* y = getRandomSortedArrayOfLength(ly);

    printIntersection(x, y, lx, ly);
    cout << endl;


//    runPrintIntersection();
//
//    crow::SimpleApp app;
//
//    std::cout << "http://0.0.0.0:18080" << std::endl;
//
//    CROW_ROUTE(app, "/")
//            ([]() {
//                return "Hello world!";
//            });
//
//    app.port(18080).run();

    return 0;
}

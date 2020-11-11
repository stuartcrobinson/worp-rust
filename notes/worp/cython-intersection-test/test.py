# Import the extension module hello.
import hello

#note - not necessary anymore cos we found SortedSets in sortedcontainers

list1 = [1, 3, 5, 7, 9, 80]
list1 = [1, 2, 3, 6, 7, 8, 80]

# Call the print_result method 
hello.print_result(list1, list2, 6, 7)




# cpdef print_result (int[] ar1, int[] ar2, m, n):
#     """This is a cpdef function that can be called from Python."""
#     print(" hi"); 
#     """
#     int i = 0, j = 0; 
#     while (i < m && j < n) 
#     { 
#         if (arr1[i] < arr2[j]) 
#             i++; 
#         else if (arr2[j] < arr1[i]) 
#             j++; 
#         else /* if arr1[i] == arr2[j] */
#         { 
#             printf(" %d ", arr2[j++]); 
#             i++; 
#         } 
#     } 
#     """

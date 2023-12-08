#include <iostream>

struct Node {
  int data;
  Node *next;
};

void swap(Node *a, Node *b) {
  int temp = a->data;
  a->data = b->data;
  b->data = temp;
}

void bubbleSort(Node *head) {
  int swapped;
  Node *ptr1;
  Node *lptr = nullptr;

  if (head == nullptr)
    return;

  do {
    swapped = 0;
    ptr1 = head;

    while (ptr1->next != lptr) {
      if (ptr1->data > ptr1->next->data) {
        swap(ptr1, ptr1->next);
        swapped = 1;
      }
      ptr1 = ptr1->next;
    }
    lptr = ptr1;
  } while (swapped);
}

void printList(Node *node) {
  while (node != nullptr) {
    std::cout << node->data << " ";
    node = node->next;
  }
  std::cout << "\n";
}

void insertAtTheBegin(Node **start_ref, int data) {
  Node *ptr1 = new Node();
  ptr1->data = data;
  ptr1->next = *start_ref;
  *start_ref = ptr1;
}

int main() {
  Node *start = nullptr;

  insertAtTheBegin(&start, 3);
  insertAtTheBegin(&start, 5);
  insertAtTheBegin(&start, 1);
  insertAtTheBegin(&start, 4);
  insertAtTheBegin(&start, 20);

  std::cout << "Linked list before sorting\n";
  printList(start);

  bubbleSort(start);

  std::cout << "\nLinked list after sorting\n";
  printList(start);

  return 0;
}

#include <iostream>

struct Node {
  int data;
  Node *next;

  Node(int data) : data(data), next(nullptr) {}
};

struct LinkedList {
  Node *head;

  LinkedList() : head(nullptr) {}

  void insert(int data) {
    Node *newNode = new Node(data);
    newNode->next = head;
    head = newNode;
  }

  void print() {
    Node *temp = head;
    while (temp != nullptr) {
      std::cout << temp->data << " ";
      temp = temp->next;
    }
    std::cout << std::endl;
  }

  void insertionSort() {
    Node *sorted = nullptr;

    Node *current = head;
    while (current != nullptr) {
      Node *next = current->next;

      Node **sortedRef = &sorted;
      while (*sortedRef != nullptr && (*sortedRef)->data < current->data) {
        sortedRef = &(*sortedRef)->next;
      }
      current->next = *sortedRef;
      *sortedRef = current;

      current = next;
    }

    head = sorted;
  }
};

int main() {
  LinkedList list;

  list.insert(5);
  list.insert(20);
  list.insert(4);
  list.insert(3);
  list.insert(30);

  std::cout << "Linked List before sorting\n";
  list.print();

  list.insertionSort();

  std::cout << "Linked List after sorting\n";
  list.print();
}

#include <iostream>

struct Node {
  int data;
  Node *next;

  Node(int data) : data(data), next(nullptr) {}
};

void swapNodesData(Node *a, Node *b) {
  int temp = a->data;
  a->data = b->data;
  b->data = temp;
}

void selectionSort(Node *head) {
  Node *start = head;
  while (start != nullptr) {
    Node *minNode = start;
    Node *current = start->next;

    while (current != nullptr) {
      if (current->data < minNode->data) {
        minNode = current;
      }
      current = current->next;
    }

    swapNodesData(start, minNode);

    start = start->next;
  }
}

void insertNode(Node *&head, int data) {
  Node *newNode = new Node(data);
  newNode->next = head;
  head = newNode;
}

void printList(Node *head) {
  Node *temp = head;
  while (temp != nullptr) {
    std::cout << temp->data << " ";
    temp = temp->next;
  }
  std::cout << "\n";
}

int main() {
  Node *head = new Node(64);
  insertNode(head, 11);
  insertNode(head, 12);
  insertNode(head, 25);
  insertNode(head, 22);

  std::cout << "Linked list before sorting:\n";
  printList(head);

  selectionSort(head);

  std::cout << "Linked list after sorting:\n";
  printList(head);

  return 0;
}

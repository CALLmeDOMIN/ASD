#include <iostream>

struct Node {
  int data;
  Node *next;

  Node() {
    this->data = 0;
    this->next = nullptr;
  }

  Node(int data) {
    this->data = data;
    this->next = nullptr;
  }
};

struct LinkedList {
  Node *head;

  LinkedList() { head = nullptr; }

  void append(int data) {
    Node *newNode = new Node(data);
    if (head == nullptr) {
      head = newNode;
      return;
    }

    Node *temp = head;
    while (temp->next != nullptr) {
      temp = temp->next;
    }
    temp->next = newNode;
  }

  void insertionSort() {
    if (head == nullptr || head->next == nullptr)
      return;

    Node *sorted = nullptr;

    Node *current = head;
    while (current != nullptr) {
      Node *next = current->next;

      sortedInsert(&sorted, current);

      current = next;
    }

    head = sorted;
  }

  void sortedInsert(Node **sortedHead, Node *newNode) {
    Node dummy;
    Node *current = &dummy;
    dummy.next = *sortedHead;

    while (current->next != nullptr && current->next->data < newNode->data) {
      current = current->next;
    }

    newNode->next = current->next;
    current->next = newNode;

    *sortedHead = dummy.next;
  }

  void printList() {
    Node *temp = head;
    while (temp != nullptr) {
      std::cout << temp->data << " ";
      temp = temp->next;
    }
    std::cout << "\n";
  }
};

int main() {
  LinkedList list;

  list.append(30);
  list.append(10);
  list.append(50);
  list.append(20);
  list.append(40);

  list.printList();

  list.insertionSort();

  list.printList();
}

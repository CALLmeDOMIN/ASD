#include <iostream>

int *tab;

void bubble_sort(int tab[], int size) {
  for (int i = 0; i < size; ++i) {
    for (int j = i; j < size; ++j) {
      if (tab[j] < tab[i]) {
        int tmp = tab[j];
        tab[j] = tab[i];
        tab[i] = tmp;
      }
    }
  }
}

void print(int tab[], int size) {
  for (int i = 0; i < size; ++i)
    std::cout << tab[i] << " ";
  std::cout << "\n";
}

int main() {
  int size;
  std::cout << "Podaj dlugosc tablicy: ";
  std::cin >> size;

  tab = new int[size];

  std::cout << "Podaj wartosci: ";
  for (int i = 0; i < size; ++i) {
    std::cin >> tab[i];
  }

  print(tab, size);

  bubble_sort(tab, size);

  print(tab, size);
}
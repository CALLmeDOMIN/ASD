#include <iostream>

int *tab;

void combSort(int tab[], int size) {
  int gap = size;
  bool swapped = true;

  while (gap > 1 || swapped) {
    gap = (gap * 10) / 13;

    if (gap < 1)
      gap = 1;

    swapped = false;

    for (int i = 0; i < size - gap; ++i) {
      if (tab[i] > tab[i + gap]) {
        std::swap(tab[i], tab[i + gap]);
        swapped = true;
      }
    }
  }
}

void print(int tab[], int size) {
  for (int i = 0; i < size; ++i) {
    std::cout << tab[i] << " ";
  }
  std::cout << "\n";
}

int main() {
  int size;
  std::cout << "Podaj dlugosc tablicy: ";
  std::cin >> size;

  tab = new int[size];

  std::cout << "Podaj elementy tablicy: ";
  for (int i = 0; i < size; ++i) {
    std::cin >> tab[i];
  }

  print(tab, size);

  combSort(tab, size);

  print(tab, size);
}
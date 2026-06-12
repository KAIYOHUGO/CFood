int main(void) {
  if (1 != 2)
    ;
  while (1 > 2)
    ;
  this_should_hoist 1 1.0;
}

void this_should_hoist(int a, float b) {}

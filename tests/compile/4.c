int main(void) {
  if (1 != 2)
    ;
  while (1 > 2)
    ;
  this_should_hoist;
}

void this_should_hoist() {}

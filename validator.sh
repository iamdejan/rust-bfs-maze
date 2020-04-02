for ((i = 1; i <= 2; i++)); do
  cargo -q run < input${i}.txt > output${i}-actual.txt
  diff output${i}-actual.txt output${i}-expected.txt
  if [[ $? -gt 0 ]]; then
    exit 1
  fi
done

exit 0

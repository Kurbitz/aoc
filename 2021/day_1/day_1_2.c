#include <stdio.h>
int main(int argc, char *argv[])
{
	if (argc != 2)
		return 1;
	FILE *input = fopen(argv[1], "r");
	int increases = -3, pos = 0, depth, sum = 0, prevSum = 0;
	int arr[3] = {0};
	while (fscanf(input, "%d\n", &depth) != EOF)
	{
		if (pos == 3)
			pos = 0;
		sum -= arr[pos];
		arr[pos] = depth;
		sum += arr[pos++];
		if (sum > prevSum)
			increases++;
		prevSum = sum;
	}
	printf("%d\n", increases);
	fclose(input);
	return 0;
}
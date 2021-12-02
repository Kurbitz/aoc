#include <stdio.h>
int main(int argc, char *argv[])
{
	// if (argc != 2)
	// 	return 1;
	argv[1] = "input.txt";
	FILE *input = fopen(argv[1], "r");
	int increases = -1;
	int depth, prevDepth = 0;
	while (fscanf(input, "%d\n", &depth) != EOF)
	{
		if (depth > prevDepth)
			increases++;
		prevDepth = depth;
	}
	printf("%d\n", increases);
	fclose(input);
	return 0;
}
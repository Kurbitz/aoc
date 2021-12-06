#include <stdio.h>
#include <math.h>
#define LEN 12

int part1(FILE *input);

int main(int argc, char *argv[])
{
	char *fName;
	if (argc == 2)
		fName = argv[1];
	else
		fName = "input.txt";
	FILE *input = fopen(fName, "r");
	if (input == NULL)
		return 2;
	
	printf("%d\n", part1(input));

	return 0;
}

int part1(FILE *input)
{
	int bitOc[LEN] = {0}, lines = 0, g = 0, e = 0;
	char line[LEN + 1];
	while (fscanf(input, "%s\n", line) != EOF)
	{
		for (int i = 0; i < LEN; i++)
			bitOc[i] += line[i] - '0';	
		lines++;
	}
	for (int i = LEN - 1, j = 0; i >= 0; i--, j++)
	{
		if (bitOc[i] > lines/2)
			g += 1*pow(2, j);
		else
			e += 1*pow(2, j);
	}
	return e*g;
}

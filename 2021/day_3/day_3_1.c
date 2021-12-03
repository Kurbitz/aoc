#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
int main()
{
	FILE *input = fopen("input.txt", "r");
	int bitOc[12] = {0}, gamma[12] = {0}, epsilon[12] = {0};
	int lines = 0, g = 0, e = 0;
	char line[13];
	while (fscanf(input, "%s\n", line) != EOF)
	{
		for (int i = 0; i < 12; i++)
			bitOc[i] += line[i] - '0';	
		lines++;
	}
	for (int i = 0; i < 12; i++)
	{
		if (bitOc[i] >= lines/2)
			gamma[i] = 1;
		else
			epsilon[i] = 1;
	}
	for (int i = 11, j = 0; i >= 0; i--, j++)
	{
		g += gamma[i]*pow(2,j);
		e += epsilon[i]*pow(2,j);
	}
	
	printf("%d\n", e*g);

	return 0;
}
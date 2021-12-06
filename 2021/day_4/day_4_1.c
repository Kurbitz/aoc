#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
	
	part1(input);
	// skapa två multidimensionella [2|3] arrayer.
	return 0;
}

int part1(FILE *input)
{
	int numbers[100] = {0};
	char line[300];
	int boards[100][5][5] = {0};
	int hitMask[100][5][5] = {0};
	const char s[1] = ",";
	fgets(line, 300, input);

	char *token;
	int i = 0;
	token = strtok(line, s);

	while (token != NULL)
	{
		numbers[i] = atoi(token);
		token = strtok(NULL, s);
		i++;
	}
	char buffer[3];
	int lines = 0;
	while (fgets(line, 20, input))
	{
		for (int i = 0; i < 5; i++)
		{
			fgets(line, 20, input);
			for (int k = 0, pos = 0; k < 5; k++)
			{
				for (int j = 0; j < 3; j++, pos++)
					buffer[j] = line[pos];
				boards[lines][i][k] = atoi(buffer);
			}
		}
		lines++;
	}
	
	

}

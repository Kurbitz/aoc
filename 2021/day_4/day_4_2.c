#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int part2(FILE *input);
int checkBoard(int board[5][5]);
int calculateScore(int board[5][5], int mask[5][5], int winningNo);

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

	printf("score: %d\n", part2(input));
	return 0;
}

int part2(FILE *input)
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
	int winning[100] = {0};
	for (int num = 0; num < 100; num++)
		for (int board = 0; board < 100; board++)
			for (int row = 0; row < 5; row++)
				for (int col = 0; col < 5; col++)
					if (boards[board][row][col] == numbers[num])
					{
						hitMask[board][row][col] = 1;
						printf("board %d\n", board);
						if (checkBoard(hitMask[board]))
						{
							if (calculateScore(boards[board], hitMask[board], numbers[num]))
								winning[board] = 1;
							int wins = 0;
							for (int w = 0; w < 100; w++)
								wins += winning[w];
							if (wins == 100)
								return calculateScore(boards[board], hitMask[board], numbers[num]);
						}
					}
	return 0;
}

int checkBoard(int board[5][5])
{
	int size = 5;
	int i, j, sum = 0;
	int result = 0;
	for (i = 0; i < size; i++)
	{
		for (j = 0; j < size; j++)
		{
			sum = sum + board[i][j];
			if (board[i][j])
				printf("\033[0;31m");			
			printf("%d ", board[i][j]);
			printf("\033[0m"); 
		}
		printf("\n");
		if (sum == 5)
			result = 1;
		sum = 0;
	}
	printf("\n");
	for (j = 0; j < size; j++)
	{
		for (i = 0; i < size; i++)
		{
			sum = sum + board[i][j];
		}
		if (sum == 5)
			result = 1;
		sum = 0;
	}
	return result;
}

int calculateScore(int board[5][5], int mask[5][5], int winningNo)
{
	int i, j, sum = 0;
	for (i = 0; i < 5; i++)
		for (j = 0; j < 5; j++)
			if (mask[i][j] == 0)
				sum += board[i][j];

	return sum * winningNo;
}
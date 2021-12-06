#include <stdio.h>

int part1()
{
	int val, cor[2] = {0};
	char direction[8];
	while (scanf("%8[a-zA-Z] %d\n", direction, &val) != EOF)
	{
		switch (direction[0])
		{
		case 'f':
			cor[0] += val;
			break;
		case 'd':
			cor[1] += val;
			break;
		case 'u':
			cor[1] -= val;
			break;
		default:
			break;
		}
	}
	return cor[0]*cor[1];
}

int main ()
{
	printf("%d", part1());
}
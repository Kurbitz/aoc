#include <stdio.h>

int part2()
{
	int aim = 0, val, cor[2] = {0};
	char direction[8];
	while (scanf("%8[a-zA-Z] %d\n", direction, &val) != EOF)
	{
		switch (direction[0])
		{
		case 'f':
			cor[0] += val;
			cor[1] += val*aim;
			break;
		case 'd':
			aim += val; break;
		case 'u':
			aim -= val; break;
		default:
			break;
		}
	}
	return cor[0]*cor[1];
}

int main ()
{
	printf("%d\n", part2());
}
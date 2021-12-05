#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#define LEN 12

typedef struct node
{
	int data[LEN];
	struct node *next;
} node;

int part2(FILE *input);
void addLast(node **list, const int *data);
void addFirst(node **list, const int *data);
static node *createListNode(const int *data);
void occurrance(node **list, int *occ);
void removeCO2Readings(node **list, int *occ, int pos, int *lines);
void removeO2Readings(node **list, int *occ, int pos, int *lines);

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

	printf("%d\n", part2(input));

	return 0;
}

int part2(FILE *input)
{
	int lines = 0, o2Lines, co2Lines, co2Val = 0, o2Val = 0;
	char line[LEN * 2];
	node *o2List = NULL, *co2List = NULL;
	int tData[LEN] = {0};

	while (fgets(line, LEN * 2, input))
	{
		for (int i = 0; i < LEN; i++)
			tData[i] = line[i] - '0';
		addLast(&o2List, tData);
		addLast(&co2List, tData);
		lines++;
	}
	co2Lines = o2Lines = lines;
	int i = 0;
	while (o2List->next != NULL)
	{
		int bitOc[LEN] = {0};
		occurrance(&o2List, bitOc);
		removeO2Readings(&o2List, bitOc, i, &o2Lines);
		i++;
	}
	i = 0;
	while (co2List->next != NULL)
	{
		int bitOc[LEN] = {0};
		occurrance(&co2List, bitOc);
		removeCO2Readings(&co2List, bitOc, i, &co2Lines);
		i++;
	}

	for (int i = LEN - 1, j = 0; i >= 0; i--, j++)
	{
		o2Val += o2List->data[i] * pow(2, j);
		co2Val += co2List->data[i] * pow(2, j);
	}
	return o2Val * co2Val;
}

void removeO2Readings(node **list, int *occ, int pos, int *lines)
{
	node *tmp = *list;
	node *prev;
	int removed = 0;
	int bitcrit = occ[pos] >= (float)(*lines) / 2;

	while (tmp != NULL && tmp->data[pos] != bitcrit)
	{
		*list = tmp->next;
		free(tmp);
		removed++;
		tmp = *list;
	}
	while (tmp != NULL)
	{
		while (tmp != NULL && tmp->data[pos] == bitcrit)
		{
			prev = tmp;
			tmp = tmp->next;
		}
		if (tmp == NULL)
			break;
		prev->next = tmp->next;
		free(tmp);
		removed++;
		tmp = tmp->next;
	}
	*lines -= removed;
}

void removeCO2Readings(node **list, int *occ, int pos, int *lines)
{
	node *tmp = *list, *prev;
	int removed = 0, bitcrit = occ[pos] < (float)(*lines) / 2;

	while (tmp != NULL && tmp->data[pos] != bitcrit)
	{
		*list = tmp->next;
		free(tmp);
		removed++;
		tmp = *list;
	}
	while (tmp != NULL)
	{
		while (tmp != NULL && tmp->data[pos] == bitcrit)
		{
			prev = tmp;
			tmp = tmp->next;
		}
		if (tmp == NULL)
			break;
		prev->next = tmp->next;
		free(tmp);
		removed++;
		tmp = tmp->next;
	}
	*lines -= removed;
}

void occurrance(node **list, int *occ)
{
	node *tmp = *list;
	while (tmp != NULL)
	{
		for (int i = 0; i < LEN; i++)
			occ[i] += tmp->data[i];
		tmp = tmp->next;
	}
}

void addFirst(node **list, const int *data)
{
	node *newNode = createListNode(data);
	if (*list == NULL)
	{
		*list = newNode;
		return;
	}
	newNode->next = *list;
	*list = newNode;
}

void addLast(node **list, const int *data)
{
	if (*list == NULL)
	{
		addFirst(list, data);
		return;
	}
	addLast(&(*list)->next, data);
}

static node *createListNode(const int *data)
{
	node *n = malloc(sizeof(node));
	if (n == NULL)
		abort();

	for (int i = 0; i < LEN; i++)
		n->data[i] = data[i];
	n->next = NULL;
	return n;
}

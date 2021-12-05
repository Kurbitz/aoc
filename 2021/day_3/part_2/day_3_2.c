#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#define LEN 12

struct node {
	int data[LEN];
	struct node *next;
} typedef node;

int part2(FILE *input);
void addLast(node **list, const int *data);
void addFirst(node **list, const int *data);
static struct node *createListNode(const int *data);
void occurrance(node **list, int *occ);
void removeCO2Readings(node **list, int *occ, int pos, int *lines);
void removeO2Readings(node **list, int *occ, int pos, int *lines);
int numberOfNodesInList(const node *list);


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
	node *o2List = NULL;
	node *co2List = NULL;
	int tData[LEN] = {0};

	while (fgets(line, LEN*2, input))
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
	return o2Val*co2Val;
}

void removeO2Readings(node **list, int *occ, int pos, int *lines)
{
	node *tmp = *list;
	node *prev;
	int removed = 0;
	int bitcrit = occ[pos] >= (float)(*lines)/2;

	

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
	node *tmp = *list;
	node *prev;
	int removed = 0;
	int bitcrit = occ[pos] < (float)(*lines)/2;	

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

void occurranceCO2(node **list, int *occ)
{
	node *tmp = *list;
	while (tmp != NULL)
	{
		for (int i = 0; i < LEN; i++)
		{
			if (tmp->data == 0)
				occ[i] += 1;
		}
		tmp = tmp->next;
	}
}

void addFirst(node **list, const int *data)
{
	node *newNode = createListNode(data);

	// om detta är första noden som läggs till i listan, sätt *list (head) till att vara nya noden
	if (*list == NULL)
	{
		*list = newNode;
		return;
	}

	// om detta inte är första, sätt den nya nodens next fält att peka på den föredetta första noden
	// Sätt *list (head) till den nya noden
	newNode->next = *list;
	*list = newNode;
}

/*Lägg till nod sist i listan
  Tips, när du hittat rätt plats kan du använda funktionen addFirst för att lägga till*/
void addLast(node **list, const int *data)
{
	node *n = *list;
	// Om listan är tom eller vi är på sista noden, kalla addFirst
	if (*list == NULL)
	{
		addFirst(list, data);
		return;
	}
	return addLast(&(*list)->next, data); // Kallas sig själv rekursivt tills sista noden är nådd
}

static struct node *createListNode(const int *data)
{
	// Allokera minne för en ny nod
	struct node *n = malloc(sizeof(struct node));
	// om minnesallokering misslyckades, avbryt programmet
	if (n == NULL)
		abort();

	// Fyll i den nya noden med rätt data och nollställ next fältet
	for (int i = 0; i < LEN; i++)
		n->data[i] = data[i];
	n->next = NULL;
	return n;
}

int numberOfNodesInList(const node *list)
{
	if (list == NULL)
		return 0;
	return 1 + numberOfNodesInList(list->next);
}
#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#define LINES 12
#define LEN 5

struct node {
	int data[LEN];
	struct node *next;
} typedef node;

int part2(FILE *input);
void addLast(node **list, const int *data);
void addFirst(node **list, const int *data);
static struct node *createListNode(const int *data);
void occurrance(node **list, int *occ);
void removeReadings(node **list, int *occ, int pos);


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
	int co2Key[LEN] = {0}, o2Key[LEN] = {0};
	int lines = 0, co2Val = 0, o2Val = 0, o2Line, co2Line;
	char line[LEN * 2];
	int arr[1000][LEN] = {0};
	node *head = NULL;
	int tData[LEN] = {0};

	while (fgets(line, LEN*2, input))
	{
		for (int i = 0; i < LEN; i++)
			tData[i] = line[i] - '0';
		addLast(&head, tData);
		lines++;
	}
	int a = 1;

	
	while (a)
	{
		for (int i = 0; i < LEN; i++)
		{
			int bitOc[LEN] = {0};
			node *tmp = head, *prev = head;
			occurrance(&head, bitOc);
			removeReadings(&head, bitOc, i);
		}
	}
	
	for (int i = LEN - 1, j = 0; i >= 0; i--, j++)
	{
		// o2Val += arr[o2Line][i] * pow(2, j);
		// co2Val += arr[co2Line][i] * pow(2, j);
	}
	printf("%d\n", o2Val*co2Val);
	return 0;
}

void removeReadings(node **list, int *occ, int pos)
{
	node *tmp = *list;
	node *prev;

	if (tmp->data[pos] != (occ[pos] >= LINES/2))
	{
		*list = tmp->next;
		free(tmp);
	}
	
	while (tmp != NULL)
	{
		if (tmp->data == (occ[pos] >= LINES/2))
		{
			prev = tmp;
			tmp = tmp->next;
		}
		else
		{
			prev->next = tmp->next;
			free(tmp);
		}
	}
}

void occurrance(node **list, int *occ)
{
	node *tmp = *list;
	while (tmp->next != NULL)
	{
		for (int i = 0; i < LEN; i++)
			occ[i] += tmp->data[i];
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
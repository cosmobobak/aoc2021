data = open("task4.txt", "r").readlines()

nums = [int(v) for v in data[0].split(",")]

boards = [list(map(lambda x: [int(v) for v in x.split()], data[x:x+5])) for x in range(2, len(data), 6)]

def is_bingo(board, marked):
    for row in board:
        if all(v in marked for v in row):
            return True
        
    for col in range(len(board[0])):
        if all(b[i][col] in marked for i in range(len(board))):
            return True
            
    return False
    
def product_unmarked(board, marked):
    acc = 0
    for row in board:
        for v in row:
            if v not in marked:
                acc += v
                
    return acc
    
marked = []
found = False
for n in nums:
    marked.append(n)
    print(marked)
    for b in boards:
        if is_bingo(b, marked):
            print(product_unmarked(b, marked) * marked[-1])
            found = True
            break
    if found:
        break
        
marked = []
won = []
for n in nums:
    marked.append(n)
    print(marked)
    all_won = True
    for b in boards:
        if b in won:
            continue
        if is_bingo(b, marked):
            won.append(b)
        all_won = False
    if all_won:
        print(product_unmarked(won[-1], marked[:-1]), marked[-2])
        print(product_unmarked(won[-1], marked[:-1]) * marked[-2])
        break
            
            
            
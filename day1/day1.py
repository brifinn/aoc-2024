import heapq
from collections import defaultdict

def main(fname):
    l1 = []
    l2 = []
    l2_counts = defaultdict(int)
    
    # gonna just assume some shit
    with open(fname) as fh:
        for line in fh:
            l1_entry, l2_entry = [int(x) for x in line.split()]
            heapq.heappush(l1, l1_entry)
            heapq.heappush(l2, l2_entry)
            l2_counts[l2_entry] += 1
    
    distance = 0
    similarity = 0
    for i in range(len(l1)):
        l1_entry = heapq.heappop(l1)
        l2_entry = heapq.heappop(l2)
        distance += abs(l1_entry - l2_entry)
        similarity += l1_entry * l2_counts[l1_entry]
    
    return distance, similarity

if __name__ == "__main__":
    print(main('input.txt'))

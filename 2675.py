retry=int(input())
for i in range(0,retry):
    ret, word=input().split()
    ret=int(ret)
    list(word)
    for j in range(1,len(word)+1):
        for k in range(0,ret):
            print(word[j-1], end='')
    print()
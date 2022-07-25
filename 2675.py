retry=int(input()) #반복할 횟수
for i in range(0,retry): 
    ret, word=input().split() #각 글자를 몇번 반복할것인지, 반복할 문자
    ret=int(ret) #숫자는 정수형으로 바꿔주기
    list(word) #문자는 리스트에 잘라서 넣기
    for j in range(1,len(word)+1): #총 글자수만큼 반복
        for k in range(0,ret): #같은글자 입력받은 횟수만큼 출력
            print(word[j-1], end='')
    print()
dial1=[['A','B','C'],['D','E','F'],['G','H','I'],['J','K','L'],['M','N','O'],['P','Q','R','S'],['T','U','V'],['W','X','Y','Z']] #다이얼 숫자
letter=list(input()) #문자 입력받아서 쪼개기
letterlen=len(letter) #총 몇문자인지 세기
finres=[] #몇초 걸리는지 기록할 리스트

for i in range(0,letterlen): # 문자 전부 다이얼 몇번에 있는지 알기위해 돌림
    th=2 #몇번째에 위치하는지 카운트
    for j in dial1: #다이얼별로 문자가 포함되어있는지 확인
        if letter[i] in j: #전부확인
            finres.append(th+1) #1초 더해서 집합에 넣기
        th+=1 #카운트 올리기

print(sum(finres)) #sum 사용해서 리스트값 다 더한값 출력
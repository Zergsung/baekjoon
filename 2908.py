num1,num2=input().split()
num1list=list(str(num1))
num2list=list(str(num2))
num1lst=[ ]
num2lst=[ ]
range=[2,1,0]

for i in range:
    num1lst.append(num1list[i])
    num2lst.append(num2list[i])
#이부분에서 문자형식의 리스트를 숫자로 바꾸는걸 모르겠어서 찾아봄
num1lst=list(map(int,num1lst))
num2lst=list(map(int,num2lst))

finnum1=num1lst[0]*100+num1lst[1]*10+num1lst[2]
finnum2=num2lst[0]*100+num2lst[1]*10+num2lst[2]

if finnum1>finnum2:
    print(finnum1)
else:
    print(finnum2)
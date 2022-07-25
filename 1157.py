word=input().lower() #소문자로만 받아서 word에 저장

wordlist=list(set(word)) #set으로 중복제거 (어떤 알파벳이있는지 확인)

countlist=[] #카운트 리스트 만들기

for i in wordlist: #알파벳 별로 단어에 몇개가 들어있는지 확인
    count = word.count(i) #알파벳이 몇개 들어있는지 카운트한걸 count함수에 저장
    countlist.append(count) #알파벳개수를 cnt리스트에 앞에서부터 저장

if countlist.count(max(countlist)) >=2: #알파벳개수중 max로 많은걸 뽑아내는데 2개이상이면 ? 출력
    print('?')

else:
    print(wordlist[(countlist.index(max(countlist)))].upper())
    #countlist에서 가장 높은수를 max로 구함. index로 몇번째에 위치하는구함. worldlist에서 그번째에 있는 알파벳을 대문자로 출력
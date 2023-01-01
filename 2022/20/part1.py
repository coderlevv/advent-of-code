def swap(mix, curr_pos, mix_pos):
    if curr_pos == mix_pos:
        return mix
    elif curr_pos < mix_pos:
        before_item = mix[:curr_pos]
        after_item = mix[curr_pos+1:]
        before_move = after_item[:mix_pos-curr_pos]
        after_move = after_item[mix_pos-curr_pos:]
        return before_item + before_move + [mix[curr_pos]] + after_move
    else:
        before_item = mix[:curr_pos]
        after_item = mix[curr_pos+1:]
        before_move = before_item[:mix_pos]
        after_move = before_item[mix_pos:]
        return before_move + [mix[curr_pos]] + after_move + after_item

if __name__ == '__main__':
    with open('input', 'r') as f:
        content = f.read()

    num = [int(n) for n in content.split('\n') if n.strip() != ""]
    res = [n * 811589153 for n in num]
    idx = list(range(len(num)))
    for _ in range(10):
        for i in range(len(idx)):
            curr_pos = idx.index(i)
            n = res[curr_pos]
            if n == 0:
                mix_pos = curr_pos
            else:
                mix_pos = (curr_pos + n) % (len(res) - 1)
                if mix_pos == 0:
                    mix_pos = len(res) - 1
            idx = swap(idx, curr_pos, mix_pos)
            res = swap(res, curr_pos, mix_pos)

    i = res.index(0)
    coord = [
    res[(i + 1000) % len(res)],
    res[(i + 2000) % len(res)],
    res[(i + 3000) % len(res)]
    ]
    print(sum(coord))

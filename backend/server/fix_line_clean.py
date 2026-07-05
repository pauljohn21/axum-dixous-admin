import sys

with open('data/migration/src/m20260701_000001_create_casbin_rule.rs', 'r') as f:
    lines = f.readlines()

out = []
i = 0
while i < len(lines):
    if '.values_panic([\n' in lines[i]:
        merged = []
        s = i
        while s < len(lines) and '])' not in lines[s]:
            merged.append(lines[s])
            s += 1
        if s < len(lines):
            merged.append(lines[s])
        content = ''.join(merged)
        start = content.find('[')
        end = content.find(']')
        if start != -1 and end != -1:
            arr_str = content[start+1:end]
            parts = []
            for item in arr_str.split('.into(),'):
                it = item.strip(' ",\n\r')
                if it:
                    parts.append(it + '.into()')
            ptype = parts[0] if len(parts) > 0 else '""'
            v0 = parts[1] if len(parts) > 1 else '""'
            v1 = parts[2] if len(parts) > 2 else '""'
            v2 = parts[3] if len(parts) > 3 else '""'
            new_line = f'            .values_panic([{ptype}, {v0}, {v1}, {v2}])\n'
            out.append(new_line)
        else:
            out.extend(merged)
        i = s + 1
    else:
        out.append(lines[i])
        i += 1

with open('data/migration/src/m20260701_000001_create_casbin_rule.rs', 'w') as f:
    f.writelines(out)
print("Fixed!")
# Kernel source as the lock check reads it: comments and the insides of string
# and character literals blanked, every offset and newline kept, so a brace or
# a lock's name in a comment or a message is never read as code.


def strip(text: str) -> str:
    out = list(text)
    i, n = 0, len(text)

    def blank(a: int, b: int) -> None:
        out[a:b] = [c if c == "\n" else " " for c in text[a:b]]

    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            blank(i, j)
            i = j
        elif text[i] == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            blank(i + 1, j)
            i = j + 1
        elif text[i] == "'" and i + 2 < n and (text[i + 2] == "'" or text[i + 1] == "\\"):
            j = text.find("'", i + 2)
            blank(i + 1, j)
            i = j + 1
        else:
            i += 1
    return "".join(out)


def close(text: str, open_at: int) -> int:
    """The `}` that closes the `{` at `open_at`."""
    depth = 0
    for j in range(open_at, len(text)):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                return j
    return len(text)


def close_of(text: str, i: int) -> int:
    """The `}` of the block that position `i` is in."""
    depth = 0
    for j in range(i, len(text)):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            if depth == 0:
                return j
            depth -= 1
    return len(text)


def line_of(text: str, at: int) -> int:
    return text.count("\n", 0, at) + 1

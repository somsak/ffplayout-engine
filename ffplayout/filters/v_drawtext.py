import os
import re

from ffplayout.utils import _text


def filter(probe, node=None):
    """
    extract title from file name and overlay it
    """
    font = ''
    source = os.path.basename(probe.src)
    match = re.match(_text.regex, source)
    title = match[1] if match else source

    if _text.fontfile and os.path.isfile(_text.fontfile):
        font = f":fontfile='{_text.fontfile}'"

    if _text.text_from_filename:
        escape = title.replace("'", "'\\\\\\''").replace("%", "\\\\\\%")
        return f"drawtext=text='{escape}':{_text.style}{font}"

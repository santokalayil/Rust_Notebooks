# pip install google
import os
import urllib

from googlesearch import search as _search


def _add_to_file(item, file_name):
    with open(file_name, 'a') as file_object:
        file_object.write(f'{item}\n')


def _run(search_query, text_file, **kwargs):
    if text_file not in os.listdir():
        with open(text_file, 'w') as fh:
            fh.write('results:\n')
            print("file successfully created!")
    else:
        print("pre_existing file found! updating the file..")

    i = 0
    for link in _search(search_query, **kwargs):
        if link not in open(text_file, 'r').readlines():
            i += 1
            print(f'{i} new results retrieved!    ', end='\r')
            _add_to_file(link, text_file)
    return


def run_search(search_query, text_file, **kwargs):
    """
    This function searches on google and takes all the links available and writes to a text file
    :param search_query:
    :param text_file:
    :param kwargs: preexisting arguments in google_search external library
    :return: 0 if no URL error else returns 1
    """
    try:
        return _run(search_query, text_file, **kwargs)
    except urllib.error.URLError as e:
        print(f"URL Error: {e}")
        return

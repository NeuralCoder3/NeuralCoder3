import requests

username = "neuralcoder3"
api_url="https://api.github.com/users/"+username+"/repos"
marker = "gh-pages"
title = "## Current Github-Pages Websites"
readme_file = "../README.md"

# retrieve all repos
repos = requests.get(api_url).json()

# filter out repos that have a gh-pages branch (has_pages = True)
gh_pages_repos = [repo for repo in repos if repo["has_pages"]]

# create a markdown list of links to the websites
links = []
for repo in gh_pages_repos:
  # hide non-public repos
  if repo["visibility"] != "public":
    continue
  url = f"https://{username}.github.io/{repo['name']}"
  if repo["homepage"] and len(repo["homepage"]) > 0:
    url = repo["homepage"]
  links.append(f"- [{repo['name']}]({repo['html_url']})")

# update the README.md
start_marker = f"<!-- {marker} start -->"
end_marker = f"<!-- {marker} end -->"
with open(readme_file, "r") as f:
  readme = f.read()
  
start = readme.find(start_marker) + len(start_marker) + 1
end = readme.find(end_marker)
content = "\n".join(links)
if title:
  content = title + "\n\n" + content
new_readme = readme[:start] + "\n" + content + "\n" + readme[end:]

with open(readme_file, "w") as f:
  f.write(new_readme)
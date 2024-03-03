from langchain_community.document_loaders import GitLoader
from git import Repo
from langchain_community.llms import Ollama
from langchain.text_splitter import RecursiveCharacterTextSplitter

repo_path = "/tmp/homelab"
# repo = Repo.clone_from(
#     "https://github.com/khuedoan/homelab", to_path=repo_path
# )

loader = GitLoader(repo_path="/tmp/homelab", branch="master")

data = loader.load()

llm = Ollama(model="mistral", document=data)

query = "What does this project do"

for chunks in llm.stream(query):
    print(chunks, end='')

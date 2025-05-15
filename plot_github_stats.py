
import matplotlib.pyplot as plt
data = {"stars": 3900, "forks": 452}
plt.bar(["Stars", "Forks"], [data["stars"], data["forks"]], color=["skyblue", "lightgreen"])
plt.title("GitHub Stats of camel-ai/camel")
plt.ylabel("Count")
plt.savefig("github_stats.png")  # 保存图片
plt.show()

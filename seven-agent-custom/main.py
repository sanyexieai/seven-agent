import json
import requests
import matplotlib.pyplot as plt
import subprocess
from bs4 import BeautifulSoup
import os
from dotenv import load_dotenv

# 加载环境变量
load_dotenv()

# 获取 API 配置
DEEPSEEK_API_KEY = os.getenv("DEEPSEEK_API_KEY")
DEEPSEEK_API_BASE_URL = os.getenv("DEEPSEEK_API_BASE_URL")

# --------------------- 自定义函数实现 ---------------------
def search_baidu(query: str, max_results: int = 5) -> dict:
    """模拟百度搜索（实际需替换为真实API或爬虫逻辑）"""
    print(f"正在搜索: {query} (模拟结果)")
    # 示例模拟数据（实际应用需替换为真实爬虫或API）
    mock_results = [
        {
            "result_id": 1,
            "title": "GitHub - camel-ai/camel: 🐫 CAMEL: 多智能体框架",
            "url": "https://github.com/camel-ai/camel"
        },
        {
            "result_id": 2,
            "title": "CAMEL框架官方文档",
            "url": "https://camel-ai.org"
        }
    ]
    return {"results": mock_results[:max_results]}

def get_github_stats(repo_url: str) -> dict:
    """使用 DeepSeek 的联网功能获取 GitHub 仓库的 Star/Fork 数据"""
    print(f"正在获取 GitHub 数据: {repo_url}")
    
    # 构建请求体
    request_body = {
        "model": "deepseek-chat",
        "messages": [
            {"role": "system", "content": "你是一个有帮助的助手，请帮我获取 GitHub 仓库的统计数据。"},
            {"role": "user", "content": f"请访问 {repo_url} 并返回该仓库的 star 和 fork 数量，以 JSON 格式返回，格式为: {{'stars': 数字, 'forks': 数字}}"}
        ],
        "stream": False
    }
    
    print("发送到 DeepSeek 的请求体:", json.dumps(request_body, ensure_ascii=False, indent=2))
    
    # 发送请求到 DeepSeek API
    headers = {
        "Authorization": f"Bearer {DEEPSEEK_API_KEY}",
        "Content-Type": "application/json"
    }
    
    try:
        print("正在发送请求到 DeepSeek API...")
        response = requests.post(
            f"{DEEPSEEK_API_BASE_URL}/chat/completions",
            headers=headers,
            json=request_body
        )
        print(f"DeepSeek API 响应状态码: {response.status_code}")
        print(f"DeepSeek API 响应内容: {response.text}")
        
        response.raise_for_status()
        result = response.json()
        
        # 从 DeepSeek 的响应中提取数据
        if "choices" in result and result["choices"]:
            content = result["choices"][0]["message"]["content"]
            print(f"DeepSeek 返回的原始内容: {content}")
            
            # 尝试从响应中提取 JSON 数据
            try:
                # 移除可能的 Markdown 代码块标记
                content = content.replace("```json", "").replace("```", "").strip()
                print(f"清理后的内容: {content}")
                
                # 直接解析 JSON
                stats = json.loads(content)
                print(f"解析后的统计数据: {stats}")
                return stats
            except json.JSONDecodeError as e:
                print(f"JSON 解析错误: {e}")
        else:
            print("DeepSeek 响应中没有找到 choices 字段")
        
        # 如果无法获取数据，返回默认值
        print("无法获取数据，返回默认值")
        return {"stars": 0, "forks": 0}
        
    except requests.exceptions.RequestException as e:
        print(f"API 请求错误: {e}")
        if hasattr(e, 'response') and e.response is not None:
            print(f"错误响应内容: {e.response.text}")
        return {"stars": 0, "forks": 0}
    except Exception as e:
        print(f"发生未知错误: {e}")
        return {"stars": 0, "forks": 0}

def save_plot_to_python_file(stats: dict):
    """将绘图代码保存为 Python 文件"""
    stars = stats.get('stars', 0)
    forks = stats.get('forks', 0)
    code = f"""
import matplotlib.pyplot as plt
data = {{"stars": {stars}, "forks": {forks}}}
plt.bar(["Stars", "Forks"], [data["stars"], data["forks"]], color=["skyblue", "lightgreen"])
plt.title("GitHub Stats of camel-ai/camel")
plt.ylabel("Count")
plt.savefig("github_stats.png")  # 保存图片
plt.show()
"""
    with open("plot_github_stats.py", "w", encoding="utf-8") as f:
        f.write(code)
    print("已生成脚本: plot_github_stats.py")

# --------------------- 处理 DeepSeek Function Calling ---------------------
def execute_tool_call(tool_call: dict):
    """执行模型请求的函数调用"""
    func_name = tool_call["function"]["name"]
    args = json.loads(tool_call["function"]["arguments"])
    
    if func_name == "search_baidu":
        return search_baidu(**args)
    elif func_name == "get_github_stats":
        return get_github_stats(**args)
    else:
        raise ValueError(f"未知函数: {func_name}")

def process_deepseek_request(user_query: str, tools: list) -> dict:
    """处理 DeepSeek 请求并返回结果"""
    # 1. 构建请求体
    request_body = {
        "model": "deepseek-chat",
        "messages": [
            {"role": "system", "content": "你是一个有帮助的助手。"},
            {"role": "user", "content": user_query}
        ],
        "tools": tools,
        "stream": False
    }
    
    # 2. 发送请求到 DeepSeek API
    headers = {
        "Authorization": f"Bearer {DEEPSEEK_API_KEY}",
        "Content-Type": "application/json"
    }
    
    try:
        print("发送请求体:", json.dumps(request_body, ensure_ascii=False, indent=2))
        response = requests.post(
            f"{DEEPSEEK_API_BASE_URL}/chat/completions",
            headers=headers,
            json=request_body
        )
        print("响应状态码:", response.status_code)
        print("响应内容:", response.text)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        print(f"API 请求错误: {e}")
        if hasattr(e.response, 'text'):
            print(f"错误详情: {e.response.text}")
        return {"content": "抱歉，API 请求失败，请稍后重试。"}
    except json.JSONDecodeError:
        print("API 响应解析错误")
        return {"content": "抱歉，API 响应格式错误，请稍后重试。"}

# --------------------- 模拟 DeepSeek 对话流程 ---------------------
def main():
    # 1. 用户提问
    user_query = "打开百度搜索，总结一下camel-ai的camel框架的github star、fork数目等，并把数字用plot包写成python文件保存到本地，并运行生成的python文件。"
    print(f"用户提问: {user_query}")

    # 2. 定义可用函数
    tools = [
        {
            "type": "function",
            "function": {
                "name": "search_baidu",
                "description": "在百度上搜索关键词并返回结果",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "搜索关键词"},
                        "max_results": {"type": "integer", "description": "返回结果数量"}
                    },
                    "required": ["query"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "get_github_stats",
                "description": "获取GitHub仓库的Star和Fork数量",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "repo_url": {"type": "string", "description": "GitHub仓库URL"}
                    },
                    "required": ["repo_url"]
                }
            }
        }
    ]

    # 3. 处理 DeepSeek 请求
    response = process_deepseek_request(user_query, tools)
    
    # 4. 执行工具调用
    if "choices" in response and response["choices"]:
        message = response["choices"][0]["message"]
        # 如果有 tool_calls，走工具调用流程
        if "tool_calls" in message and message["tool_calls"]:
            for tool_call in message["tool_calls"]:
                result = execute_tool_call(tool_call)
                print(f"工具调用结果: {result}")
                func_name = tool_call["function"]["name"]
                if func_name == "search_baidu":
                    github_url = None
                    for result_item in result["results"]:
                        if "github.com" in result_item["url"]:
                            github_url = result_item["url"]
                            print("找到 github_url:", github_url)
                            break
                    if github_url:
                        # 发起第二次请求获取 GitHub 数据
                        stats_response = process_deepseek_request(
                            f"获取 {github_url} 的 star 和 fork 数据",
                            tools
                        )
                        if "choices" in stats_response and stats_response["choices"]:
                            stats_message = stats_response["choices"][0]["message"]
                            if "tool_calls" in stats_message and stats_message["tool_calls"]:
                                stats_result = execute_tool_call(stats_message["tool_calls"][0])
                                print("GitHub 数据:", stats_result)
                                save_plot_to_python_file(stats_result)
                                print("\n运行生成的脚本...")
                                subprocess.run(["python", "plot_github_stats.py"], check=True)
                            elif "content" in stats_message and stats_message["content"]:
                                print("助手回复:", stats_message["content"])
                            else:
                                print("无有效回复内容。")
                        else:
                            print("API 响应异常：", stats_response)
                elif func_name == "get_github_stats":
                    print("GitHub 数据:", result)
                    save_plot_to_python_file(result)
                    print("\n运行生成的脚本...")
                    subprocess.run(["python", "plot_github_stats.py"], check=True)
        elif "content" in message and message["content"]:
            print("助手回复:", message["content"])
        else:
            print("无有效回复内容。")
    else:
        print("API 响应异常：", response)

if __name__ == "__main__":
    main()
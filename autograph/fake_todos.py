#!/usr/bin/env python3

import json


def main() -> None:
    todos = [
        {
            "id": "00000000-0000-0000-0000-000000000001",
            "title": "Review sidecar demo",
            "description": "This todo comes from Python sidecar",
            "deadline": None,
            "completed": False,
            "project_id": None,
        },
        {
            "id": "00000000-0000-0000-0000-000000000002",
            "title": "Prepare fake todos",
            "description": "No project assigned",
            "deadline": None,
            "completed": True,
            "project_id": None,
        },
        {
            "id": "00000000-0000-0000-0000-000000000003",
            "title": "Try python backend",
            "description": "Demo data for inbox",
            "deadline": None,
            "completed": False,
            "project_id": None,
        },
    ]
    print(json.dumps(todos))


if __name__ == "__main__":
    main()

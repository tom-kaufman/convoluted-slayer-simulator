import os
import random

import polars as pl
from flask import Flask
from flask_restful import Resource, Api


app = Flask(__name__)
api = Api(app)

df = None
total_task_weight = None
n = None
tasks_csv_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'tasks.csv')


class SlayerMaster(Resource):
    def get(self):
        global df
        global total_task_weight
        global n
        if df is None:
            df = pl.read_csv(tasks_csv_path)
        if not total_task_weight:
            total_task_weight = df['weight'].sum()
        if n is None:
            n = len(df)
        task_monster = SlayerMaster.choose_task(df, random.randint(1, n))
        task_size = SlayerMaster.choose_task_size(df, task_monster)
        return {'monster': task_monster, 'size': task_size}

    @staticmethod
    def choose_task(df, value):
        smaller_rows = df.filter(pl.col("weight_running_total") < value)
        return df[len(smaller_rows), 'monster_id']

    @staticmethod
    def choose_task_size(df, monster_id):
        return random.randint(
            df[monster_id, 'amount_min'],
            df[monster_id, 'amount_max']
        )


api.add_resource(SlayerMaster, '/')


if __name__ == '__main__':
    app.run(debug=True)

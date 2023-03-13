import os
import random

import polars as pl
from flask import Flask
from flask_restful import Resource, Api


app = Flask(__name__)
api = Api(app)

df = None
monster_csv_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'slayer_monster.csv')


class SlayerMonster(Resource):
    def get(self, monster_id):
        global df
        if df is None:
            df = pl.read_csv(monster_csv_path)
        xp = SlayerMonster.get_xp(df, monster_id)
        return {'xp': xp}

    @staticmethod
    def get_xp(df, monster_id):
        return df[monster_id, 'xp']


api.add_resource(SlayerMonster, '/<int:monster_id>')


if __name__ == '__main__':
    app.run(debug=True)


def create_app():
    return app

package com.jackchristensen.www.conwaysgameoflife;

import android.os.Handler;
import android.support.v4.app.Fragment;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.TextView;

import java.util.concurrent.ThreadLocalRandom;

import go.cgol.Cgol;

/**
 * A placeholder fragment containing a simple view.
 */
public class MainActivityFragment extends Fragment {

    private Cgol.World world;
    private Handler handler = new Handler();

    public MainActivityFragment() {
    }

    @Override
    public void onStart() {
        super.onStart();
        world = Cgol.NewWorld(30, 30);
        for(int i = 0; i < 100; i++) {
            int x = ThreadLocalRandom.current().nextInt(0, (int) world.Width());
            int y = ThreadLocalRandom.current().nextInt(0, (int) world.Height());
            world.Set(x, y, true);
        }

        handler.postDelayed(runnable, 250);
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState) {
        return inflater.inflate(R.layout.fragment_main, container, false);
    }

    private void render() {
        StringBuilder sb = new StringBuilder();
        for(int y = 0; y < world.Height(); y++) {
            for(int x = 0; x < world.Width(); x++) {
                if(world.Get(x, y)) {
                    sb.append("*");
                } else {
                    sb.append(" ");
                }
            }
            sb.append("\n");
        }

        TextView worldTextBox = (TextView) getView().findViewById(R.id.worldTextView);
        worldTextBox.setText(sb);
    }

    private Runnable runnable = new Runnable() {
        @Override
        public void run() {
            world = world.Step();
            render();
            handler.postDelayed(this, 250);
        }
    };
}
